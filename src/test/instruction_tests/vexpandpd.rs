use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vexpandpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 137, 136, 196], OperandSize::Dword)
}

fn vexpandpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 679038171, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 136, 180, 241, 219, 76, 121, 40], OperandSize::Dword)
}

fn vexpandpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 66, 253, 143, 136, 238], OperandSize::Qword)
}

fn vexpandpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 137, 136, 20, 86], OperandSize::Qword)
}

fn vexpandpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 173, 136, 205], OperandSize::Dword)
}

fn vexpandpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 1606732239, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 172, 136, 132, 67, 207, 201, 196, 95], OperandSize::Dword)
}

fn vexpandpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 253, 172, 136, 217], OperandSize::Qword)
}

fn vexpandpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(YMM26)), operand2: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 253, 169, 136, 17], OperandSize::Qword)
}

fn vexpandpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 136, 202], OperandSize::Dword)
}

fn vexpandpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 863108513, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 136, 164, 207, 161, 253, 113, 51], OperandSize::Dword)
}

fn vexpandpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 34, 253, 207, 136, 247], OperandSize::Qword)
}

fn vexpandpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(ZMM26)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1691008629, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 253, 201, 136, 20, 189, 117, 190, 202, 100], OperandSize::Qword)
}

