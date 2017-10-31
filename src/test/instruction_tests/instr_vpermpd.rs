use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 174, 22, 215], OperandSize::Dword)
}

#[test]
fn vpermpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 174, 22, 20, 187], OperandSize::Dword)
}

#[test]
fn vpermpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 245, 188, 22, 7], OperandSize::Dword)
}

#[test]
fn vpermpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 130, 245, 163, 22, 238], OperandSize::Qword)
}

#[test]
fn vpermpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM20)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 221, 164, 22, 27], OperandSize::Qword)
}

#[test]
fn vpermpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 137205392, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 182, 22, 20, 189, 144, 150, 45, 8], OperandSize::Qword)
}

#[test]
fn vpermpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 205, 22, 255], OperandSize::Dword)
}

#[test]
fn vpermpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 139729129, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 202, 22, 12, 141, 233, 24, 84, 8], OperandSize::Dword)
}

#[test]
fn vpermpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 606046581, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 220, 22, 156, 78, 117, 137, 31, 36], OperandSize::Dword)
}

#[test]
fn vpermpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM26)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 130, 173, 195, 22, 197], OperandSize::Qword)
}

#[test]
fn vpermpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 628199319, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 189, 203, 22, 140, 80, 151, 143, 113, 37], OperandSize::Qword)
}

#[test]
fn vpermpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 133, 210, 22, 44, 118], OperandSize::Qword)
}

#[test]
fn vpermpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 1, 198, 98], OperandSize::Dword)
}

#[test]
fn vpermpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 502478618, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 1, 36, 69, 26, 55, 243, 29, 116], OperandSize::Dword)
}

#[test]
fn vpermpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 1, 243, 122], OperandSize::Qword)
}

#[test]
fn vpermpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM1)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 1, 11, 51], OperandSize::Qword)
}

#[test]
fn vpermpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 175, 1, 205, 122], OperandSize::Dword)
}

#[test]
fn vpermpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(ESI, EBX, Two, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 173, 1, 36, 94, 44], OperandSize::Dword)
}

#[test]
fn vpermpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 1548632443, Some(OperandSize::Qword), None)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 190, 1, 52, 85, 123, 65, 78, 92, 103], OperandSize::Dword)
}

#[test]
fn vpermpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM29)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 19, 253, 169, 1, 205, 53], OperandSize::Qword)
}

#[test]
fn vpermpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 1930907588, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 172, 1, 28, 77, 196, 79, 23, 115, 90], OperandSize::Qword)
}

#[test]
fn vpermpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM17)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(106)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 227, 253, 187, 1, 12, 223, 106], OperandSize::Qword)
}

#[test]
fn vpermpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 206, 1, 246, 102], OperandSize::Dword)
}

#[test]
fn vpermpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectDisplaced(ESI, 1838371965, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 207, 1, 158, 125, 84, 147, 109, 41], OperandSize::Dword)
}

#[test]
fn vpermpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 217, 1, 28, 255, 26], OperandSize::Dword)
}

#[test]
fn vpermpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM9)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 195, 253, 207, 1, 209, 26], OperandSize::Qword)
}

#[test]
fn vpermpd_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 817059719, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 206, 1, 44, 117, 135, 87, 179, 48, 4], OperandSize::Qword)
}

#[test]
fn vpermpd_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM17)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 227, 253, 219, 1, 12, 208, 80], OperandSize::Qword)
}

