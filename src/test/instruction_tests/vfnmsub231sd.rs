use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfnmsub231sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 191, 233], OperandSize::Dword)
}

fn vfnmsub231sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 191, 4, 155], OperandSize::Dword)
}

fn vfnmsub231sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 191, 231], OperandSize::Qword)
}

fn vfnmsub231sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 191, 40], OperandSize::Qword)
}

fn vfnmsub231sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 223, 191, 209], OperandSize::Dword)
}

fn vfnmsub231sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1004813887, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 141, 191, 52, 141, 63, 62, 228, 59], OperandSize::Dword)
}

fn vfnmsub231sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 194, 173, 190, 191, 222], OperandSize::Qword)
}

fn vfnmsub231sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 215532330, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 131, 191, 180, 208, 42, 195, 216, 12], OperandSize::Qword)
}

