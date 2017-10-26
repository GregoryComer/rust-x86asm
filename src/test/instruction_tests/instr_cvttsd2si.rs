use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvttsd2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 44, 254], OperandSize::Dword)
}

#[test]
fn cvttsd2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(EBP)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 44, 46], OperandSize::Dword)
}

#[test]
fn cvttsd2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 44, 230], OperandSize::Qword)
}

#[test]
fn cvttsd2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 632687635, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 44, 180, 86, 19, 12, 182, 37], OperandSize::Qword)
}

#[test]
fn cvttsd2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(RBX)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 44, 221], OperandSize::Qword)
}

#[test]
fn cvttsd2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTTSD2SI, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 682955108, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 44, 140, 216, 100, 17, 181, 40], OperandSize::Qword)
}

