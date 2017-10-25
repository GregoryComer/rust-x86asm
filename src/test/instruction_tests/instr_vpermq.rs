use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 171, 54, 238], OperandSize::Dword)
}

#[test]
fn vpermq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EDX, 1571117450, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 169, 54, 178, 138, 89, 165, 93], OperandSize::Dword)
}

#[test]
fn vpermq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 788053736, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 187, 54, 148, 146, 232, 190, 248, 46], OperandSize::Dword)
}

#[test]
fn vpermq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 82, 149, 170, 54, 255], OperandSize::Qword)
}

#[test]
fn vpermq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 237, 166, 54, 44, 176], OperandSize::Qword)
}

#[test]
fn vpermq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM31)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 133, 181, 54, 48], OperandSize::Qword)
}

#[test]
fn vpermq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 204, 54, 214], OperandSize::Dword)
}

#[test]
fn vpermq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1367291649, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 213, 207, 54, 60, 157, 1, 55, 127, 81], OperandSize::Dword)
}

#[test]
fn vpermq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 1739129490, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 222, 54, 148, 150, 146, 2, 169, 103], OperandSize::Dword)
}

#[test]
fn vpermq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 197, 205, 54, 230], OperandSize::Qword)
}

#[test]
fn vpermq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Eight, 1368793293, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 253, 206, 54, 172, 247, 205, 32, 150, 81], OperandSize::Qword)
}

#[test]
fn vpermq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectDisplaced(RCX, 2062888898, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 157, 222, 54, 153, 194, 47, 245, 122], OperandSize::Qword)
}

#[test]
fn vpermq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 0, 213, 84], OperandSize::Dword)
}

#[test]
fn vpermq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 0, 44, 152, 107], OperandSize::Dword)
}

#[test]
fn vpermq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 0, 238, 25], OperandSize::Qword)
}

#[test]
fn vpermq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(RSI, 642434285, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 0, 158, 237, 196, 74, 38, 91], OperandSize::Qword)
}

#[test]
fn vpermq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 174, 0, 213, 35], OperandSize::Dword)
}

#[test]
fn vpermq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 172, 0, 33, 71], OperandSize::Dword)
}

#[test]
fn vpermq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(EBX, 1924704999, Some(OperandSize::Qword), None)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 191, 0, 179, 231, 170, 184, 114, 107], OperandSize::Dword)
}

#[test]
fn vpermq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 115, 253, 171, 0, 243, 86], OperandSize::Qword)
}

#[test]
fn vpermq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM26)), operand2: Some(IndirectDisplaced(RBX, 2079213324, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 99, 253, 175, 0, 147, 12, 71, 238, 123, 86], OperandSize::Qword)
}

#[test]
fn vpermq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(RDX, 1710032173, Some(OperandSize::Qword), None)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 188, 0, 170, 45, 5, 237, 101, 14], OperandSize::Qword)
}

#[test]
fn vpermq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 202, 0, 232, 91], OperandSize::Dword)
}

#[test]
fn vpermq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexed(ESI, ESI, Four, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 207, 0, 28, 182, 65], OperandSize::Dword)
}

#[test]
fn vpermq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 702203566, Some(OperandSize::Qword), None)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 220, 0, 164, 86, 174, 198, 218, 41, 94], OperandSize::Dword)
}

#[test]
fn vpermq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM10)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 211, 253, 206, 0, 226, 9], OperandSize::Qword)
}

#[test]
fn vpermq_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM13)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 115, 253, 206, 0, 44, 64, 91], OperandSize::Qword)
}

#[test]
fn vpermq_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM11)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 253, 221, 0, 27, 76], OperandSize::Qword)
}

