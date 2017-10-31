use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 237, 239], OperandSize::Dword)
}

#[test]
fn paddsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 237, 60, 176], OperandSize::Dword)
}

#[test]
fn paddsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 237, 197], OperandSize::Qword)
}

#[test]
fn paddsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 290591897, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 237, 164, 143, 153, 20, 82, 17], OperandSize::Qword)
}

#[test]
fn paddsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 237, 203], OperandSize::Dword)
}

#[test]
fn paddsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 237, 47], OperandSize::Dword)
}

#[test]
fn paddsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 237, 194], OperandSize::Qword)
}

#[test]
fn paddsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 1506334718, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 237, 60, 77, 254, 215, 200, 89], OperandSize::Qword)
}

