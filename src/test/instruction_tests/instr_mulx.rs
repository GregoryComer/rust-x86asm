use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn mulx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 75, 246, 223], OperandSize::Dword)
}

#[test]
fn mulx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 83, 246, 32], OperandSize::Dword)
}

#[test]
fn mulx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESP)), operand3: Some(Direct(ESI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 91, 246, 214], OperandSize::Qword)
}

#[test]
fn mulx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1040380242, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 75, 246, 28, 85, 82, 241, 2, 62], OperandSize::Qword)
}

#[test]
fn mulx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(RSI)), operand2: Some(Direct(RBX)), operand3: Some(Direct(RDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 227, 246, 247], OperandSize::Qword)
}

#[test]
fn mulx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MULX, operand1: Some(Direct(RSP)), operand2: Some(Direct(RSI)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 203, 246, 36, 147], OperandSize::Qword)
}

