use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmaddsub132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 150, 209], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 150, 23], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 150, 246], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 150, 2], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 150, 238], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 150, 20, 81], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 150, 239], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 987784197, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 150, 180, 74, 5, 100, 224, 58], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 137, 150, 238], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 229, 140, 150, 62], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ECX, 679190817, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 156, 150, 177, 33, 161, 123, 40], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 157, 138, 150, 232], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Four, 1386437057, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 157, 137, 150, 164, 135, 193, 89, 163, 82], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 155, 150, 26], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 169, 150, 230], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 172, 150, 48], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 499481174, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 190, 150, 20, 149, 86, 122, 197, 29], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM23)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 197, 161, 150, 197], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 171, 150, 59], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectDisplaced(RBX, 724474980, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 189, 187, 150, 187, 100, 156, 46, 43], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 219, 150, 203], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1010412464, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 203, 150, 44, 125, 176, 171, 57, 60], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 222, 150, 48], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM9)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 66, 181, 247, 150, 225], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1511578037, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 157, 204, 150, 36, 141, 181, 217, 24, 90], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 245, 217, 150, 4, 177], OperandSize::Qword)
}

