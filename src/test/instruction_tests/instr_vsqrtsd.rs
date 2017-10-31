use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsqrtsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 81, 249], OperandSize::Dword)
}

#[test]
fn vsqrtsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 1060087713, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 81, 188, 112, 161, 167, 47, 63], OperandSize::Dword)
}

#[test]
fn vsqrtsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 81, 222], OperandSize::Qword)
}

#[test]
fn vsqrtsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1751561436, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 81, 12, 117, 220, 180, 102, 104], OperandSize::Qword)
}

#[test]
fn vsqrtsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 207, 187, 81, 238], OperandSize::Dword)
}

#[test]
fn vsqrtsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1187804056, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 215, 140, 81, 12, 141, 152, 115, 204, 70], OperandSize::Dword)
}

#[test]
fn vsqrtsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 231, 190, 81, 196], OperandSize::Qword)
}

#[test]
fn vsqrtsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Two, 583697531, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 215, 140, 81, 132, 72, 123, 132, 202, 34], OperandSize::Qword)
}

