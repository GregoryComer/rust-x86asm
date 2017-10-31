use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vandps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 84, 242], OperandSize::Dword)
}

#[test]
fn vandps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 84, 26], OperandSize::Dword)
}

#[test]
fn vandps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 84, 210], OperandSize::Qword)
}

#[test]
fn vandps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1073650080, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 84, 44, 205, 160, 153, 254, 63], OperandSize::Qword)
}

#[test]
fn vandps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 84, 202], OperandSize::Dword)
}

#[test]
fn vandps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDX, 703273032, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 84, 170, 72, 24, 235, 41], OperandSize::Dword)
}

#[test]
fn vandps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 84, 199], OperandSize::Qword)
}

#[test]
fn vandps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RSI, 1492380973, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 84, 190, 45, 237, 243, 88], OperandSize::Qword)
}

#[test]
fn vandps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 76, 140, 84, 226], OperandSize::Dword)
}

#[test]
fn vandps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 935487737, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 76, 139, 84, 4, 245, 249, 104, 194, 55], OperandSize::Dword)
}

#[test]
fn vandps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 92, 153, 84, 60, 139], OperandSize::Dword)
}

#[test]
fn vandps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 68, 141, 84, 247], OperandSize::Qword)
}

#[test]
fn vandps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectDisplaced(RDX, 1643265645, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 36, 130, 84, 138, 109, 62, 242, 97], OperandSize::Qword)
}

#[test]
fn vandps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectDisplaced(RAX, 531088202, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 20, 158, 84, 152, 74, 195, 167, 31], OperandSize::Qword)
}

#[test]
fn vandps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 68, 170, 84, 250], OperandSize::Dword)
}

#[test]
fn vandps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 116, 175, 84, 0], OperandSize::Dword)
}

#[test]
fn vandps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EBX, 1964528462, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 108, 185, 84, 131, 78, 83, 24, 117], OperandSize::Dword)
}

#[test]
fn vandps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM18)), operand3: Some(Direct(YMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 193, 108, 164, 84, 244], OperandSize::Qword)
}

#[test]
fn vandps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 394492813, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 36, 164, 84, 148, 154, 141, 123, 131, 23], OperandSize::Qword)
}

#[test]
fn vandps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 516217628, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 60, 187, 84, 156, 246, 28, 219, 196, 30], OperandSize::Qword)
}

#[test]
fn vandps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 68, 201, 84, 238], OperandSize::Dword)
}

#[test]
fn vandps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(EDI, 1942636012, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 116, 207, 84, 167, 236, 69, 202, 115], OperandSize::Dword)
}

#[test]
fn vandps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EDX, 1307520783, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 92, 219, 84, 162, 15, 47, 239, 77], OperandSize::Dword)
}

#[test]
fn vandps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 193, 84, 205, 84, 223], OperandSize::Qword)
}

#[test]
fn vandps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1337389699, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 28, 194, 84, 36, 253, 131, 242, 182, 79], OperandSize::Qword)
}

#[test]
fn vandps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1369023210, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 84, 211, 84, 28, 181, 234, 162, 153, 81], OperandSize::Qword)
}

