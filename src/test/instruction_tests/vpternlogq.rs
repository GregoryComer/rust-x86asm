use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpternlogq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 205, 138, 37, 234, 66], OperandSize::Dword)
}

fn vpternlogq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 805129180, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(6)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 229, 142, 37, 60, 253, 220, 75, 253, 47, 6], OperandSize::Dword)
}

fn vpternlogq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1618750917, Some(OperandSize::Qword), None)), operand4: Some(Literal8(111)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 245, 159, 37, 44, 117, 197, 45, 124, 96, 111], OperandSize::Dword)
}

fn vpternlogq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM24)), operand4: Some(Literal8(68)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 147, 157, 135, 37, 192, 68], OperandSize::Qword)
}

fn vpternlogq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 634478296, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 245, 142, 37, 44, 181, 216, 94, 209, 37, 101], OperandSize::Qword)
}

fn vpternlogq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM30)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 141, 147, 37, 50, 66], OperandSize::Qword)
}

fn vpternlogq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(107)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 221, 169, 37, 241, 107], OperandSize::Dword)
}

fn vpternlogq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 1637646577, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 237, 174, 37, 140, 86, 241, 128, 156, 97, 1], OperandSize::Dword)
}

fn vpternlogq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 24362376, Some(OperandSize::Qword), None)), operand4: Some(Literal8(49)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 205, 186, 37, 12, 189, 136, 189, 115, 1, 49], OperandSize::Dword)
}

fn vpternlogq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM27)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 131, 133, 161, 37, 203, 125], OperandSize::Qword)
}

fn vpternlogq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM27)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 165, 167, 37, 39, 3], OperandSize::Qword)
}

fn vpternlogq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 563686377, Some(OperandSize::Qword), None)), operand4: Some(Literal8(30)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 189, 180, 37, 132, 128, 233, 43, 153, 33, 30], OperandSize::Qword)
}

fn vpternlogq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM2)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 197, 202, 37, 210, 75], OperandSize::Dword)
}

fn vpternlogq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 236063511, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 221, 207, 37, 172, 119, 23, 11, 18, 14, 116], OperandSize::Dword)
}

fn vpternlogq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 645286049, Some(OperandSize::Qword), None)), operand4: Some(Literal8(15)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 221, 223, 37, 36, 221, 161, 72, 118, 38, 15], OperandSize::Dword)
}

fn vpternlogq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(ZMM13)), operand4: Some(Literal8(111)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 211, 149, 199, 37, 253, 111], OperandSize::Qword)
}

fn vpternlogq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1419267143, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(63)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 115, 141, 201, 37, 28, 141, 71, 76, 152, 84, 63], OperandSize::Qword)
}

fn vpternlogq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM20)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(7)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 227, 221, 212, 37, 9, 7], OperandSize::Qword)
}

