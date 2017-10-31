use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 233, 223], OperandSize::Dword)
}

#[test]
fn psubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(MM4)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 233, 33], OperandSize::Dword)
}

#[test]
fn psubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 233, 250], OperandSize::Qword)
}

#[test]
fn psubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(MM5)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 233, 43], OperandSize::Qword)
}

#[test]
fn psubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 233, 238], OperandSize::Dword)
}

#[test]
fn psubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 233, 33], OperandSize::Dword)
}

#[test]
fn psubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 233, 246], OperandSize::Qword)
}

#[test]
fn psubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RDX, 154592018, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 233, 138, 18, 227, 54, 9], OperandSize::Qword)
}

