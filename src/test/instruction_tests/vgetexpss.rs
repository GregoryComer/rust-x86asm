use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vgetexpss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 159, 67, 220], OperandSize::Dword)
}

fn vgetexpss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ESI, 1293418976, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 137, 67, 174, 224, 1, 24, 77], OperandSize::Dword)
}

fn vgetexpss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 45, 158, 67, 249], OperandSize::Qword)
}

fn vgetexpss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSS, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectDisplaced(RBX, 1864668857, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 37, 137, 67, 179, 185, 150, 36, 111], OperandSize::Qword)
}

