use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmsub213sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 171, 232], OperandSize::Dword)
}

fn vfmsub213sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 171, 33], OperandSize::Dword)
}

fn vfmsub213sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 171, 251], OperandSize::Qword)
}

fn vfmsub213sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RAX, 740266477, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 171, 144, 237, 145, 31, 44], OperandSize::Qword)
}

fn vfmsub213sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 185, 171, 240], OperandSize::Dword)
}

fn vfmsub213sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(ESI, 1362542598, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 171, 150, 6, 192, 54, 81], OperandSize::Dword)
}

fn vfmsub213sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 2, 149, 177, 171, 224], OperandSize::Qword)
}

fn vfmsub213sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213SD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 197, 143, 171, 27], OperandSize::Qword)
}

