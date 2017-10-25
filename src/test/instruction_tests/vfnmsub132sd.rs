use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfnmsub132sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 159, 222], OperandSize::Dword)
}

fn vfnmsub132sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 159, 28, 190], OperandSize::Dword)
}

fn vfnmsub132sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 159, 229], OperandSize::Qword)
}

fn vfnmsub132sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 159, 24], OperandSize::Qword)
}

fn vfnmsub132sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 156, 159, 242], OperandSize::Dword)
}

fn vfnmsub132sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 257512214, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 229, 141, 159, 52, 157, 22, 83, 89, 15], OperandSize::Dword)
}

fn vfnmsub132sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 245, 255, 159, 229], OperandSize::Qword)
}

fn vfnmsub132sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132SD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectDisplaced(RBX, 1065981366, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 157, 137, 159, 131, 182, 149, 137, 63], OperandSize::Qword)
}

