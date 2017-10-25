use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmaddsub132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 150, 217], OperandSize::Dword)
}

fn vfmaddsub132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 356008773, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 150, 36, 189, 69, 67, 56, 21], OperandSize::Dword)
}

fn vfmaddsub132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 150, 255], OperandSize::Qword)
}

fn vfmaddsub132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RDX, 664863265, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 150, 162, 33, 2, 161, 39], OperandSize::Qword)
}

fn vfmaddsub132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 150, 202], OperandSize::Dword)
}

fn vfmaddsub132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Two, 969624790, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 150, 188, 89, 214, 76, 203, 57], OperandSize::Dword)
}

fn vfmaddsub132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 150, 219], OperandSize::Qword)
}

fn vfmaddsub132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 150, 41], OperandSize::Qword)
}

fn vfmaddsub132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 141, 150, 200], OperandSize::Dword)
}

fn vfmaddsub132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 761655116, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 141, 150, 60, 133, 76, 239, 101, 45], OperandSize::Dword)
}

fn vfmaddsub132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 100544566, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 159, 150, 172, 150, 54, 48, 254, 5], OperandSize::Dword)
}

fn vfmaddsub132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 34, 157, 132, 150, 230], OperandSize::Qword)
}

fn vfmaddsub132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1626203040, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 221, 137, 150, 44, 141, 160, 227, 237, 96], OperandSize::Qword)
}

fn vfmaddsub132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RBX, 1522010331, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 253, 153, 150, 179, 219, 8, 184, 90], OperandSize::Qword)
}

fn vfmaddsub132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 174, 150, 202], OperandSize::Dword)
}

fn vfmaddsub132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1849013552, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 174, 150, 44, 149, 48, 181, 53, 110], OperandSize::Dword)
}

fn vfmaddsub132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(ECX, 1566088557, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 187, 150, 185, 109, 157, 88, 93], OperandSize::Dword)
}

fn vfmaddsub132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 213, 171, 150, 232], OperandSize::Qword)
}

fn vfmaddsub132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 213, 172, 150, 60, 246], OperandSize::Qword)
}

fn vfmaddsub132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 717695372, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 213, 188, 150, 44, 245, 140, 41, 199, 42], OperandSize::Qword)
}

fn vfmaddsub132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 186, 150, 226], OperandSize::Dword)
}

fn vfmaddsub132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1778745391, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 206, 150, 4, 245, 47, 128, 5, 106], OperandSize::Dword)
}

fn vfmaddsub132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 205, 220, 150, 30], OperandSize::Dword)
}

fn vfmaddsub132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 189, 151, 150, 223], OperandSize::Qword)
}

fn vfmaddsub132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1426211158, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 197, 196, 150, 20, 77, 86, 65, 2, 85], OperandSize::Qword)
}

fn vfmaddsub132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectDisplaced(RCX, 2014100971, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 181, 213, 150, 169, 235, 189, 12, 120], OperandSize::Qword)
}

