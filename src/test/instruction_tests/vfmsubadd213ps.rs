use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmsubadd213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 167, 217], OperandSize::Dword)
}

fn vfmsubadd213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EDX, 1395183925, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 167, 130, 53, 209, 40, 83], OperandSize::Dword)
}

fn vfmsubadd213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 167, 242], OperandSize::Qword)
}

fn vfmsubadd213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 167, 22], OperandSize::Qword)
}

fn vfmsubadd213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 167, 223], OperandSize::Dword)
}

fn vfmsubadd213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 167, 40], OperandSize::Dword)
}

fn vfmsubadd213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 167, 209], OperandSize::Qword)
}

fn vfmsubadd213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 167, 60, 210], OperandSize::Qword)
}

fn vfmsubadd213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 143, 167, 237], OperandSize::Dword)
}

fn vfmsubadd213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 753502687, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 139, 167, 20, 85, 223, 137, 233, 44], OperandSize::Dword)
}

fn vfmsubadd213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Four, 2143347812, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 159, 167, 140, 155, 100, 228, 192, 127], OperandSize::Dword)
}

fn vfmsubadd213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 37, 132, 167, 214], OperandSize::Qword)
}

fn vfmsubadd213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM29)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 21, 130, 167, 6], OperandSize::Qword)
}

fn vfmsubadd213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 53, 155, 167, 12, 194], OperandSize::Qword)
}

fn vfmsubadd213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 117, 170, 167, 247], OperandSize::Dword)
}

fn vfmsubadd213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 1419543649, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 172, 167, 140, 152, 97, 132, 156, 84], OperandSize::Dword)
}

fn vfmsubadd213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 883441600, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 185, 167, 164, 178, 192, 63, 168, 52], OperandSize::Dword)
}

fn vfmsubadd213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 2, 117, 169, 167, 217], OperandSize::Qword)
}

fn vfmsubadd213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 59586105, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 85, 162, 167, 52, 133, 57, 54, 141, 3], OperandSize::Qword)
}

fn vfmsubadd213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 189, 167, 52, 150], OperandSize::Qword)
}

fn vfmsubadd213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 153, 167, 249], OperandSize::Dword)
}

fn vfmsubadd213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(ESI, 1451585183, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 203, 167, 166, 159, 110, 133, 86], OperandSize::Dword)
}

fn vfmsubadd213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 219, 167, 24], OperandSize::Dword)
}

fn vfmsubadd213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 178, 69, 253, 167, 214], OperandSize::Qword)
}

fn vfmsubadd213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1112786663, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 109, 193, 167, 28, 125, 231, 198, 83, 66], OperandSize::Qword)
}

fn vfmsubadd213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM15)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 5, 222, 167, 35], OperandSize::Qword)
}

