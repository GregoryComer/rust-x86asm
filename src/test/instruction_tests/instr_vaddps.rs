use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaddps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 88, 248], OperandSize::Dword)
}

#[test]
fn vaddps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 507698715, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 88, 156, 75, 27, 222, 66, 30], OperandSize::Dword)
}

#[test]
fn vaddps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 88, 207], OperandSize::Qword)
}

#[test]
fn vaddps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RSI, 1562552879, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 88, 174, 47, 170, 34, 93], OperandSize::Qword)
}

#[test]
fn vaddps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 88, 241], OperandSize::Dword)
}

#[test]
fn vaddps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1017228861, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 88, 4, 93, 61, 174, 161, 60], OperandSize::Dword)
}

#[test]
fn vaddps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 88, 214], OperandSize::Qword)
}

#[test]
fn vaddps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RDI, 929871602, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 88, 175, 242, 182, 108, 55], OperandSize::Qword)
}

#[test]
fn vaddps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 84, 142, 88, 221], OperandSize::Dword)
}

#[test]
fn vaddps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 924733037, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 68, 141, 88, 28, 245, 109, 78, 30, 55], OperandSize::Dword)
}

#[test]
fn vaddps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Two, 1316142599, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 76, 159, 88, 132, 121, 7, 190, 114, 78], OperandSize::Dword)
}

#[test]
fn vaddps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 193, 76, 139, 88, 205], OperandSize::Qword)
}

#[test]
fn vaddps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1147538107, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 92, 143, 88, 36, 77, 187, 10, 102, 68], OperandSize::Qword)
}

#[test]
fn vaddps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 84, 151, 88, 28, 95], OperandSize::Qword)
}

#[test]
fn vaddps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 68, 170, 88, 211], OperandSize::Dword)
}

#[test]
fn vaddps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 269147774, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 76, 169, 88, 52, 117, 126, 222, 10, 16], OperandSize::Dword)
}

#[test]
fn vaddps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 68, 187, 88, 60, 135], OperandSize::Dword)
}

#[test]
fn vaddps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 129, 84, 167, 88, 235], OperandSize::Qword)
}

#[test]
fn vaddps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 12, 162, 88, 44, 176], OperandSize::Qword)
}

#[test]
fn vaddps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 108, 181, 88, 4, 150], OperandSize::Qword)
}

#[test]
fn vaddps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 187, 88, 219], OperandSize::Dword)
}

#[test]
fn vaddps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(EAX, 354601478, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 116, 207, 88, 144, 6, 202, 34, 21], OperandSize::Dword)
}

#[test]
fn vaddps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(ESI, 15609197, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 76, 219, 88, 190, 109, 45, 238, 0], OperandSize::Dword)
}

#[test]
fn vaddps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(ZMM18)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 161, 12, 147, 88, 210], OperandSize::Qword)
}

#[test]
fn vaddps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 268962725, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 44, 205, 88, 20, 253, 165, 11, 8, 16], OperandSize::Qword)
}

#[test]
fn vaddps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 57171023, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 97, 36, 215, 88, 132, 182, 79, 92, 104, 3], OperandSize::Qword)
}

