use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmsub231sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 187, 214], OperandSize::Dword)
}

fn vfmsub231sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 187, 2], OperandSize::Dword)
}

fn vfmsub231sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 187, 194], OperandSize::Qword)
}

fn vfmsub231sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 453031607, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 187, 4, 213, 183, 182, 0, 27], OperandSize::Qword)
}

fn vfmsub231sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 249, 187, 209], OperandSize::Dword)
}

fn vfmsub231sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 974385254, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 143, 187, 180, 137, 102, 240, 19, 58], OperandSize::Dword)
}

fn vfmsub231sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 82, 229, 177, 187, 213], OperandSize::Qword)
}

fn vfmsub231sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectDisplaced(RDX, 1707401603, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 237, 134, 187, 154, 131, 225, 196, 101], OperandSize::Qword)
}

