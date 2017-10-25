use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtuqq2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 254, 139, 122, 198], OperandSize::Dword)
}

fn vcvtuqq2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 254, 139, 122, 28, 83], OperandSize::Dword)
}

fn vcvtuqq2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 17, 254, 140, 122, 223], OperandSize::Qword)
}

fn vcvtuqq2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(XMM12)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 254, 142, 122, 36, 211], OperandSize::Qword)
}

fn vcvtuqq2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 254, 175, 122, 209], OperandSize::Dword)
}

fn vcvtuqq2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(EAX, 1405721196, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 254, 175, 122, 160, 108, 154, 201, 83], OperandSize::Dword)
}

fn vcvtuqq2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 161, 254, 171, 122, 227], OperandSize::Qword)
}

fn vcvtuqq2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(YMM15)), operand2: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 254, 170, 122, 62], OperandSize::Qword)
}

fn vcvtuqq2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 254, 250, 122, 205], OperandSize::Dword)
}

fn vcvtuqq2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectDisplaced(EDI, 828888431, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 254, 201, 122, 167, 111, 213, 103, 49], OperandSize::Dword)
}

fn vcvtuqq2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM26)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 1, 254, 250, 122, 242], OperandSize::Qword)
}

fn vcvtuqq2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 1907147186, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 254, 204, 122, 12, 133, 178, 193, 172, 113], OperandSize::Qword)
}

