use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmaddsub231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 182, 200], OperandSize::Dword)
}

fn vfmaddsub231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 182, 20, 179], OperandSize::Dword)
}

fn vfmaddsub231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 182, 207], OperandSize::Qword)
}

fn vfmaddsub231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 182, 32], OperandSize::Qword)
}

fn vfmaddsub231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 182, 197], OperandSize::Dword)
}

fn vfmaddsub231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 1044723737, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 182, 172, 159, 25, 56, 69, 62], OperandSize::Dword)
}

fn vfmaddsub231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 182, 207], OperandSize::Qword)
}

fn vfmaddsub231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 182, 60, 192], OperandSize::Qword)
}

fn vfmaddsub231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 77, 138, 182, 239], OperandSize::Dword)
}

fn vfmaddsub231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 137, 182, 42], OperandSize::Dword)
}

fn vfmaddsub231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDX, 2061020716, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 157, 182, 138, 44, 174, 216, 122], OperandSize::Dword)
}

fn vfmaddsub231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 37, 141, 182, 251], OperandSize::Qword)
}

fn vfmaddsub231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 770137605, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 53, 135, 182, 148, 80, 5, 94, 231, 45], OperandSize::Qword)
}

fn vfmaddsub231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 61, 148, 182, 44, 217], OperandSize::Qword)
}

fn vfmaddsub231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 175, 182, 196], OperandSize::Dword)
}

fn vfmaddsub231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 930725529, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 175, 182, 164, 241, 153, 190, 121, 55], OperandSize::Dword)
}

fn vfmaddsub231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EDX, 804698188, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 77, 187, 182, 146, 76, 184, 246, 47], OperandSize::Dword)
}

fn vfmaddsub231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 194, 21, 161, 182, 231], OperandSize::Qword)
}

fn vfmaddsub231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 69, 170, 182, 26], OperandSize::Qword)
}

fn vfmaddsub231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM26)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 45, 179, 182, 9], OperandSize::Qword)
}

fn vfmaddsub231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 187, 182, 226], OperandSize::Dword)
}

fn vfmaddsub231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 1596112007, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 205, 182, 148, 159, 135, 188, 34, 95], OperandSize::Dword)
}

fn vfmaddsub231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(ECX, 1940738055, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 218, 182, 185, 7, 80, 173, 115], OperandSize::Dword)
}

fn vfmaddsub231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 53, 251, 182, 240], OperandSize::Qword)
}

fn vfmaddsub231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 2085069612, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 45, 207, 182, 12, 157, 44, 163, 71, 124], OperandSize::Qword)
}

fn vfmaddsub231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 21, 221, 182, 52, 211], OperandSize::Qword)
}

