use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfixupimmss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 77, 158, 85, 220, 41], OperandSize::Dword)
}

fn vfixupimmss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EDI, 497252143, Some(OperandSize::Dword), None)), operand4: Some(Literal8(96)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 85, 138, 85, 175, 47, 119, 163, 29, 96], OperandSize::Dword)
}

fn vfixupimmss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 53, 149, 85, 200, 103], OperandSize::Qword)
}

fn vfixupimmss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 969579381, Some(OperandSize::Dword), None)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 227, 13, 143, 85, 188, 75, 117, 155, 202, 57, 39], OperandSize::Qword)
}

