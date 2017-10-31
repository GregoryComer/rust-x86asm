use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaddsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 88, 247], OperandSize::Dword)
}

#[test]
fn vaddsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1942752977, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 88, 12, 181, 209, 14, 204, 115], OperandSize::Dword)
}

#[test]
fn vaddsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 88, 241], OperandSize::Qword)
}

#[test]
fn vaddsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 88, 31], OperandSize::Qword)
}

#[test]
fn vaddsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 223, 254, 88, 234], OperandSize::Dword)
}

#[test]
fn vaddsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 355398939, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 255, 140, 88, 4, 93, 27, 245, 46, 21], OperandSize::Dword)
}

#[test]
fn vaddsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 159, 183, 88, 227], OperandSize::Qword)
}

#[test]
fn vaddsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 215, 129, 88, 20, 78], OperandSize::Qword)
}

