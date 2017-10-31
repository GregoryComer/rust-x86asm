use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub231ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 187, 242], OperandSize::Dword)
}

#[test]
fn vfmsub231ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 187, 52, 138], OperandSize::Dword)
}

#[test]
fn vfmsub231ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 187, 243], OperandSize::Qword)
}

#[test]
fn vfmsub231ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 187, 4, 118], OperandSize::Qword)
}

#[test]
fn vfmsub231ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 255, 187, 220], OperandSize::Dword)
}

#[test]
fn vfmsub231ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 673234424, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 77, 141, 187, 36, 181, 248, 189, 32, 40], OperandSize::Dword)
}

#[test]
fn vfmsub231ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 18, 69, 210, 187, 241], OperandSize::Qword)
}

#[test]
fn vfmsub231ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 1268978296, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 85, 133, 187, 164, 138, 120, 18, 163, 75], OperandSize::Qword)
}

