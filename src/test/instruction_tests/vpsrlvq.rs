use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsrlvq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 69, 211], OperandSize::Dword)
}

fn vpsrlvq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 35632791, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 69, 60, 245, 151, 182, 31, 2], OperandSize::Dword)
}

fn vpsrlvq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 69, 253], OperandSize::Qword)
}

fn vpsrlvq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 69, 0], OperandSize::Qword)
}

fn vpsrlvq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 69, 247], OperandSize::Dword)
}

fn vpsrlvq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 69, 19], OperandSize::Dword)
}

fn vpsrlvq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 69, 207], OperandSize::Qword)
}

fn vpsrlvq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 2087066637, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 69, 36, 221, 13, 28, 102, 124], OperandSize::Qword)
}

fn vpsrlvq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 143, 69, 255], OperandSize::Dword)
}

fn vpsrlvq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EBX, 89920256, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 142, 69, 139, 0, 19, 92, 5], OperandSize::Dword)
}

fn vpsrlvq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDX, 1050912717, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 154, 69, 186, 205, 167, 163, 62], OperandSize::Dword)
}

fn vpsrlvq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 237, 130, 69, 192], OperandSize::Qword)
}

fn vpsrlvq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 1791587268, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 141, 130, 69, 148, 201, 196, 115, 201, 106], OperandSize::Qword)
}

fn vpsrlvq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 237, 146, 69, 52, 80], OperandSize::Qword)
}

fn vpsrlvq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 170, 69, 216], OperandSize::Dword)
}

fn vpsrlvq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 169, 69, 20, 193], OperandSize::Dword)
}

fn vpsrlvq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1506905463, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 245, 187, 69, 60, 157, 119, 141, 209, 89], OperandSize::Dword)
}

fn vpsrlvq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 165, 173, 69, 240], OperandSize::Qword)
}

fn vpsrlvq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 173, 164, 69, 20, 70], OperandSize::Qword)
}

fn vpsrlvq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 205, 180, 69, 12, 126], OperandSize::Qword)
}

fn vpsrlvq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 202, 69, 205], OperandSize::Dword)
}

fn vpsrlvq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Eight, 1680607612, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 207, 69, 164, 210, 124, 9, 44, 100], OperandSize::Dword)
}

fn vpsrlvq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EDI, 686597069, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 220, 69, 167, 205, 163, 236, 40], OperandSize::Dword)
}

fn vpsrlvq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM14)), operand3: Some(Direct(ZMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 162, 141, 201, 69, 200], OperandSize::Qword)
}

fn vpsrlvq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 245, 203, 69, 36, 247], OperandSize::Qword)
}

fn vpsrlvq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectDisplaced(RCX, 1097378455, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 141, 210, 69, 137, 151, 170, 104, 65], OperandSize::Qword)
}

