use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvttsd2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 44, 219], OperandSize::Dword)
}

#[test]
fn cvttsd2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 1268805266, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 44, 140, 218, 146, 110, 160, 75], OperandSize::Dword)
}

#[test]
fn cvttsd2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 44, 233], OperandSize::Qword)
}

#[test]
fn cvttsd2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(RDI, RDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 44, 36, 255], OperandSize::Qword)
}

#[test]
fn cvttsd2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(RBX)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 44, 223], OperandSize::Qword)
}

#[test]
fn cvttsd2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 44, 28, 193], OperandSize::Qword)
}

