use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfixupimmsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(115)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 237, 154, 85, 239, 115], OperandSize::Dword)
}

fn vfixupimmsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 1540894690, Some(OperandSize::Qword), None)), operand4: Some(Literal8(71)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 237, 142, 85, 172, 90, 226, 47, 216, 91, 71], OperandSize::Dword)
}

fn vfixupimmsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM24)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 147, 197, 151, 85, 216, 91], OperandSize::Qword)
}

fn vfixupimmsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMSD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectDisplaced(RAX, 1072326570, Some(OperandSize::Qword), None)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 227, 165, 133, 85, 168, 170, 103, 234, 63, 108], OperandSize::Qword)
}

