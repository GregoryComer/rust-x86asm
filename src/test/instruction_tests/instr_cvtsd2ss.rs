use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtsd2ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 90, 216], OperandSize::Dword)
}

#[test]
fn cvtsd2ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 2123315284, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 90, 28, 189, 84, 56, 143, 126], OperandSize::Dword)
}

#[test]
fn cvtsd2ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 90, 194], OperandSize::Qword)
}

#[test]
fn cvtsd2ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RBX, 1833285246, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 90, 147, 126, 182, 69, 109], OperandSize::Qword)
}

