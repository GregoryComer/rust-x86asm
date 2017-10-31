use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmaddsub132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 150, 204], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 150, 52, 147], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 150, 231], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 150, 7], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 150, 248], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDX, 834967243, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 150, 162, 203, 150, 196, 49], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 150, 203], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1092697547, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 150, 12, 157, 203, 61, 33, 65], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 150, 200], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 142, 150, 12, 242], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 1760964626, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 117, 153, 150, 164, 177, 18, 48, 246, 104], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 146, 117, 137, 150, 232], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 1869163568, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 29, 129, 150, 156, 191, 48, 44, 105, 111], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 1387924500, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 53, 153, 150, 140, 191, 20, 12, 186, 82], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 117, 173, 150, 200], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EDX, 1462498385, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 173, 150, 130, 81, 244, 43, 87], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 191, 150, 3], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 101, 169, 150, 225], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RBX, 1382470495, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 101, 169, 150, 171, 95, 211, 102, 82], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RSI, 1622044425, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 101, 189, 150, 158, 9, 111, 174, 96], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 223, 150, 194], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Two, 1238260936, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 150, 164, 123, 200, 92, 206, 73], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 217, 150, 51], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 82, 109, 222, 150, 232], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 109, 204, 150, 17], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 608845416, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 101, 209, 150, 20, 117, 104, 62, 74, 36], OperandSize::Qword)
}

