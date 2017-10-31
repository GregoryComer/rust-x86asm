use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phaddw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 1, 222], OperandSize::Dword)
}

#[test]
fn phaddw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1913368367, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 1, 52, 189, 47, 175, 11, 114], OperandSize::Dword)
}

#[test]
fn phaddw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 1, 255], OperandSize::Qword)
}

#[test]
fn phaddw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1445943484, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 1, 36, 141, 188, 88, 47, 86], OperandSize::Qword)
}

#[test]
fn phaddw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 1, 192], OperandSize::Dword)
}

#[test]
fn phaddw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 1, 62], OperandSize::Dword)
}

#[test]
fn phaddw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 1, 192], OperandSize::Qword)
}

#[test]
fn phaddw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 658103155, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 1, 36, 157, 115, 219, 57, 39], OperandSize::Qword)
}

