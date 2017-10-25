use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vunpcklps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 20, 229], OperandSize::Dword)
}

fn vunpcklps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1477707184, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 20, 28, 77, 176, 5, 20, 88], OperandSize::Dword)
}

fn vunpcklps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 20, 255], OperandSize::Qword)
}

fn vunpcklps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 20, 60, 87], OperandSize::Qword)
}

fn vunpcklps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 20, 245], OperandSize::Dword)
}

fn vunpcklps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1403456409, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 20, 36, 149, 153, 11, 167, 83], OperandSize::Dword)
}

fn vunpcklps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 20, 202], OperandSize::Qword)
}

fn vunpcklps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 903223758, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 20, 132, 208, 206, 25, 214, 53], OperandSize::Qword)
}

fn vunpcklps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 116, 137, 20, 218], OperandSize::Dword)
}

fn vunpcklps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 137, 20, 36, 150], OperandSize::Dword)
}

fn vunpcklps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 895327947, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 116, 159, 20, 188, 150, 203, 158, 93, 53], OperandSize::Dword)
}

fn vunpcklps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 68, 130, 20, 233], OperandSize::Qword)
}

fn vunpcklps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 12, 138, 20, 4, 193], OperandSize::Qword)
}

fn vunpcklps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1552668939, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 100, 147, 20, 36, 221, 11, 217, 139, 92], OperandSize::Qword)
}

fn vunpcklps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 173, 20, 231], OperandSize::Dword)
}

fn vunpcklps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 207618956, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 92, 171, 20, 44, 133, 140, 3, 96, 12], OperandSize::Dword)
}

fn vunpcklps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 124, 191, 20, 36, 223], OperandSize::Dword)
}

fn vunpcklps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 17, 68, 172, 20, 244], OperandSize::Qword)
}

fn vunpcklps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 27368445, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 108, 167, 20, 36, 205, 253, 155, 161, 1], OperandSize::Qword)
}

fn vunpcklps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1460896343, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 12, 189, 20, 36, 253, 87, 130, 19, 87], OperandSize::Qword)
}

fn vunpcklps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 108, 205, 20, 192], OperandSize::Dword)
}

fn vunpcklps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(ESI, 29010894, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 92, 202, 20, 158, 206, 171, 186, 1], OperandSize::Dword)
}

fn vunpcklps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 68, 219, 20, 52, 248], OperandSize::Dword)
}

fn vunpcklps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 124, 195, 20, 249], OperandSize::Qword)
}

fn vunpcklps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 648621963, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 28, 197, 20, 140, 152, 139, 47, 169, 38], OperandSize::Qword)
}

fn vunpcklps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectDisplaced(RSI, 1288802122, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 97, 68, 212, 20, 150, 74, 143, 209, 76], OperandSize::Qword)
}

