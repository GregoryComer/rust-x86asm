use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsrlvd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 69, 212], OperandSize::Dword)
}

fn vpsrlvd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 355218726, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 69, 172, 222, 38, 53, 44, 21], OperandSize::Dword)
}

fn vpsrlvd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 69, 203], OperandSize::Qword)
}

fn vpsrlvd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RDI, 375637076, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 69, 135, 84, 196, 99, 22], OperandSize::Qword)
}

fn vpsrlvd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 69, 254], OperandSize::Dword)
}

fn vpsrlvd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1379683026, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 69, 52, 93, 210, 74, 60, 82], OperandSize::Dword)
}

fn vpsrlvd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 69, 232], OperandSize::Qword)
}

fn vpsrlvd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 69, 20, 78], OperandSize::Qword)
}

fn vpsrlvd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 69, 234], OperandSize::Dword)
}

fn vpsrlvd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDX, 208675857, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 140, 69, 138, 17, 36, 112, 12], OperandSize::Dword)
}

fn vpsrlvd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDX, 1214935061, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 154, 69, 138, 21, 112, 106, 72], OperandSize::Dword)
}

fn vpsrlvd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 13, 140, 69, 193], OperandSize::Qword)
}

fn vpsrlvd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 69980273, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 53, 143, 69, 36, 205, 113, 208, 43, 4], OperandSize::Qword)
}

fn vpsrlvd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1476600687, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 53, 149, 69, 36, 125, 111, 35, 3, 88], OperandSize::Qword)
}

fn vpsrlvd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 175, 69, 251], OperandSize::Dword)
}

fn vpsrlvd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 686637267, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 169, 69, 172, 142, 211, 64, 237, 40], OperandSize::Dword)
}

fn vpsrlvd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 1166298909, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 190, 69, 180, 65, 29, 79, 132, 69], OperandSize::Dword)
}

fn vpsrlvd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 172, 69, 244], OperandSize::Qword)
}

fn vpsrlvd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 2087780396, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 109, 167, 69, 164, 147, 44, 0, 113, 124], OperandSize::Qword)
}

fn vpsrlvd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM21)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 85, 180, 69, 18], OperandSize::Qword)
}

fn vpsrlvd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 69, 250], OperandSize::Dword)
}

fn vpsrlvd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 203, 69, 44, 177], OperandSize::Dword)
}

fn vpsrlvd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 218, 69, 44, 91], OperandSize::Dword)
}

fn vpsrlvd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 66, 101, 206, 69, 235], OperandSize::Qword)
}

fn vpsrlvd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 53, 196, 69, 36, 193], OperandSize::Qword)
}

fn vpsrlvd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(RCX, 825582126, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 101, 221, 69, 137, 46, 98, 53, 49], OperandSize::Qword)
}

