use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 233, 211], OperandSize::Dword)
}

#[test]
fn psubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(MM3)), operand2: Some(IndirectDisplaced(ECX, 1519979196, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 233, 153, 188, 10, 153, 90], OperandSize::Dword)
}

#[test]
fn psubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 233, 207], OperandSize::Qword)
}

#[test]
fn psubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 129116886, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 233, 180, 121, 214, 42, 178, 7], OperandSize::Qword)
}

#[test]
fn psubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 233, 210], OperandSize::Dword)
}

#[test]
fn psubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(ECX, EDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 233, 60, 185], OperandSize::Dword)
}

#[test]
fn psubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 233, 238], OperandSize::Qword)
}

#[test]
fn psubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 233, 28, 178], OperandSize::Qword)
}

