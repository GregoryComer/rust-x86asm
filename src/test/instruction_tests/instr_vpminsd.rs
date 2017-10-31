use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 57, 224], OperandSize::Dword)
}

#[test]
fn vpminsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 57, 12, 178], OperandSize::Dword)
}

#[test]
fn vpminsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 57, 201], OperandSize::Qword)
}

#[test]
fn vpminsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RAX, 1463294719, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 57, 160, 255, 26, 56, 87], OperandSize::Qword)
}

#[test]
fn vpminsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 57, 213], OperandSize::Dword)
}

#[test]
fn vpminsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 57, 33], OperandSize::Dword)
}

#[test]
fn vpminsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 57, 207], OperandSize::Qword)
}

#[test]
fn vpminsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RBX, 1749633864, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 57, 155, 72, 75, 73, 104], OperandSize::Qword)
}

#[test]
fn vpminsd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 143, 57, 210], OperandSize::Dword)
}

#[test]
fn vpminsd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 142, 57, 49], OperandSize::Dword)
}

#[test]
fn vpminsd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 1804859217, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 159, 57, 180, 214, 81, 247, 147, 107], OperandSize::Dword)
}

#[test]
fn vpminsd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 2, 13, 132, 57, 195], OperandSize::Qword)
}

#[test]
fn vpminsd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM9)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 53, 142, 57, 54], OperandSize::Qword)
}

#[test]
fn vpminsd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM10)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 45, 153, 57, 57], OperandSize::Qword)
}

#[test]
fn vpminsd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 171, 57, 208], OperandSize::Dword)
}

#[test]
fn vpminsd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 170, 57, 32], OperandSize::Dword)
}

#[test]
fn vpminsd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 186, 57, 47], OperandSize::Dword)
}

#[test]
fn vpminsd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 109, 171, 57, 200], OperandSize::Qword)
}

#[test]
fn vpminsd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectDisplaced(RSI, 1052943945, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 21, 163, 57, 158, 73, 166, 194, 62], OperandSize::Qword)
}

#[test]
fn vpminsd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM28)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 29, 180, 57, 62], OperandSize::Qword)
}

#[test]
fn vpminsd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 101, 201, 57, 223], OperandSize::Dword)
}

#[test]
fn vpminsd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 1157140370, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 57, 148, 78, 146, 143, 248, 68], OperandSize::Dword)
}

#[test]
fn vpminsd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 110384133, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 219, 57, 172, 80, 5, 84, 148, 6], OperandSize::Dword)
}

#[test]
fn vpminsd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 77, 195, 57, 241], OperandSize::Qword)
}

#[test]
fn vpminsd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM14)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 13, 201, 57, 16], OperandSize::Qword)
}

#[test]
fn vpminsd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(RBX, 2028002796, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 69, 222, 57, 147, 236, 221, 224, 120], OperandSize::Qword)
}

