use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpalignr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(38)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 15, 246, 38], OperandSize::Dword)
}

fn vpalignr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EBX, 1118206633, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(68)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 15, 179, 169, 122, 166, 66, 68], OperandSize::Dword)
}

fn vpalignr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(23)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 15, 216, 23], OperandSize::Qword)
}

fn vpalignr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(11)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 15, 3, 11], OperandSize::Qword)
}

fn vpalignr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 15, 242, 108], OperandSize::Dword)
}

fn vpalignr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 1084956005, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 15, 148, 87, 101, 29, 171, 64, 87], OperandSize::Dword)
}

fn vpalignr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 15, 245, 97], OperandSize::Qword)
}

fn vpalignr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RSI, 591816611, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 15, 158, 163, 103, 70, 35, 124], OperandSize::Qword)
}

fn vpalignr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(32)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 69, 140, 15, 230, 32], OperandSize::Dword)
}

fn vpalignr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(112)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 101, 142, 15, 52, 158, 112], OperandSize::Dword)
}

fn vpalignr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(33)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 115, 13, 131, 15, 234, 33], OperandSize::Qword)
}

fn vpalignr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(79)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 99, 69, 137, 15, 10, 79], OperandSize::Qword)
}

fn vpalignr_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 77, 171, 15, 230, 37], OperandSize::Dword)
}

fn vpalignr_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 173, 15, 35, 13], OperandSize::Dword)
}

fn vpalignr_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM30)), operand4: Some(Literal8(4)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 147, 21, 167, 15, 254, 4], OperandSize::Qword)
}

fn vpalignr_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1569633044, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 99, 29, 169, 15, 60, 133, 20, 179, 142, 93, 69], OperandSize::Qword)
}

fn vpalignr_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM5)), operand4: Some(Literal8(20)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 77, 201, 15, 221, 20], OperandSize::Dword)
}

fn vpalignr_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(ESI, 232343754, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 117, 206, 15, 134, 202, 72, 217, 13, 83], OperandSize::Dword)
}

fn vpalignr_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM26)), operand4: Some(Literal8(27)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 19, 117, 193, 15, 218, 27], OperandSize::Qword)
}

fn vpalignr_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1136938775, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 37, 202, 15, 52, 181, 23, 79, 196, 67, 41], OperandSize::Qword)
}

