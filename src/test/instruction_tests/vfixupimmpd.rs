use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfixupimmpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(20)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 229, 139, 84, 218, 20], OperandSize::Dword)
}

fn vfixupimmpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 277777, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 205, 138, 84, 60, 245, 17, 61, 4, 0, 70], OperandSize::Dword)
}

fn vfixupimmpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 1106198102, Some(OperandSize::Qword), None)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 213, 155, 84, 188, 81, 86, 62, 239, 65, 18], OperandSize::Dword)
}

fn vfixupimmpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM22)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 51, 221, 132, 84, 230, 65], OperandSize::Qword)
}

fn vfixupimmpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(45)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 99, 253, 139, 84, 12, 134, 45], OperandSize::Qword)
}

fn vfixupimmpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Qword), None)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 99, 253, 147, 84, 28, 192, 51], OperandSize::Qword)
}

fn vfixupimmpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(53)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 205, 170, 84, 202, 53], OperandSize::Dword)
}

fn vfixupimmpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 862955797, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 205, 172, 84, 44, 253, 21, 169, 111, 51, 8], OperandSize::Dword)
}

fn vfixupimmpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Qword), None)), operand4: Some(Literal8(68)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 229, 189, 84, 12, 147, 68], OperandSize::Dword)
}

fn vfixupimmpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM21)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 163, 181, 170, 84, 205, 28], OperandSize::Qword)
}

fn vfixupimmpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Two, 178833175, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 99, 181, 166, 84, 132, 115, 23, 199, 168, 10, 34], OperandSize::Qword)
}

fn vfixupimmpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectDisplaced(RAX, 1089633347, Some(OperandSize::Qword), None)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 227, 189, 179, 84, 184, 67, 124, 242, 64, 87], OperandSize::Qword)
}

fn vfixupimmpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(112)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 159, 84, 224, 112], OperandSize::Dword)
}

fn vfixupimmpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Eight, 537096575, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(76)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 205, 206, 84, 132, 203, 127, 113, 3, 32, 76], OperandSize::Dword)
}

fn vfixupimmpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Two, 978006597, Some(OperandSize::Qword), None)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 205, 220, 84, 188, 71, 69, 50, 75, 58, 116], OperandSize::Dword)
}

fn vfixupimmpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM10)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 67, 181, 153, 84, 234, 108], OperandSize::Qword)
}

fn vfixupimmpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM15)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(29)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 99, 133, 204, 84, 42, 29], OperandSize::Qword)
}

fn vfixupimmpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 1746430631, Some(OperandSize::Qword), None)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 141, 211, 84, 172, 134, 167, 106, 24, 104, 118], OperandSize::Qword)
}

