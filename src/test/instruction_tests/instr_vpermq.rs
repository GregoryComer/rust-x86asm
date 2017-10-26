use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 171, 54, 201], OperandSize::Dword)
}

#[test]
fn vpermq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 1019028777, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 171, 54, 156, 191, 41, 37, 189, 60], OperandSize::Dword)
}

#[test]
fn vpermq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(ESI, 491749441, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 186, 54, 134, 65, 128, 79, 29], OperandSize::Dword)
}

#[test]
fn vpermq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 82, 221, 166, 54, 233], OperandSize::Qword)
}

#[test]
fn vpermq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 237, 167, 54, 52, 91], OperandSize::Qword)
}

#[test]
fn vpermq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 197, 189, 54, 59], OperandSize::Qword)
}

#[test]
fn vpermq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 205, 54, 211], OperandSize::Dword)
}

#[test]
fn vpermq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EBX, 193993204, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 202, 54, 147, 244, 25, 144, 11], OperandSize::Dword)
}

#[test]
fn vpermq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 219, 54, 1], OperandSize::Dword)
}

#[test]
fn vpermq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 221, 201, 54, 246], OperandSize::Qword)
}

#[test]
fn vpermq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 2017327293, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 141, 196, 54, 28, 221, 189, 248, 61, 120], OperandSize::Qword)
}

#[test]
fn vpermq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 229, 221, 54, 54], OperandSize::Qword)
}

#[test]
fn vpermq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 0, 215, 62], OperandSize::Dword)
}

#[test]
fn vpermq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 900346207, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 0, 148, 64, 95, 49, 170, 53, 80], OperandSize::Dword)
}

#[test]
fn vpermq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 0, 250, 18], OperandSize::Qword)
}

#[test]
fn vpermq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(RCX, 2795851, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 0, 153, 75, 169, 42, 0, 88], OperandSize::Qword)
}

#[test]
fn vpermq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 172, 0, 197, 93], OperandSize::Dword)
}

#[test]
fn vpermq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 169, 0, 52, 206, 112], OperandSize::Dword)
}

#[test]
fn vpermq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(EAX, 428044929, Some(OperandSize::Qword), None)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 185, 0, 128, 129, 114, 131, 25, 105], OperandSize::Dword)
}

#[test]
fn vpermq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM25)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 3, 253, 169, 0, 217, 4], OperandSize::Qword)
}

#[test]
fn vpermq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM12)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 808478162, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 115, 253, 172, 0, 36, 77, 210, 101, 48, 48, 53], OperandSize::Qword)
}

#[test]
fn vpermq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM8)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Four, 809446605, Some(OperandSize::Qword), None)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 115, 253, 188, 0, 132, 184, 205, 44, 63, 48, 57], OperandSize::Qword)
}

#[test]
fn vpermq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 204, 0, 220, 48], OperandSize::Dword)
}

#[test]
fn vpermq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM6)), operand2: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 207, 0, 50, 54], OperandSize::Dword)
}

#[test]
fn vpermq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1034328962, Some(OperandSize::Qword), None)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 221, 0, 60, 125, 130, 155, 166, 61, 71], OperandSize::Dword)
}

#[test]
fn vpermq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM22)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 163, 253, 207, 0, 206, 92], OperandSize::Qword)
}

#[test]
fn vpermq_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 82060067, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 201, 0, 164, 203, 35, 35, 228, 4, 109], OperandSize::Qword)
}

#[test]
fn vpermq_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM12)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Eight, 328944091, Some(OperandSize::Qword), None)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 253, 223, 0, 164, 217, 219, 73, 155, 19, 53], OperandSize::Qword)
}

