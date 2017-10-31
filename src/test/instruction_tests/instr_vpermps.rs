use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 22, 195], OperandSize::Dword)
}

#[test]
fn vpermps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1556863247, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 22, 44, 181, 15, 217, 203, 92], OperandSize::Dword)
}

#[test]
fn vpermps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 22, 250], OperandSize::Qword)
}

#[test]
fn vpermps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RCX, 1024465231, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 22, 177, 79, 25, 16, 61], OperandSize::Qword)
}

#[test]
fn vpermps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 173, 22, 202], OperandSize::Dword)
}

#[test]
fn vpermps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 175, 22, 4, 114], OperandSize::Dword)
}

#[test]
fn vpermps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1576937559, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 190, 22, 44, 149, 87, 40, 254, 93], OperandSize::Dword)
}

#[test]
fn vpermps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 21, 165, 22, 208], OperandSize::Qword)
}

#[test]
fn vpermps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 85, 172, 22, 12, 248], OperandSize::Qword)
}

#[test]
fn vpermps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM31)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 5, 178, 22, 58], OperandSize::Qword)
}

#[test]
fn vpermps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 22, 236], OperandSize::Dword)
}

#[test]
fn vpermps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 207, 22, 46], OperandSize::Dword)
}

#[test]
fn vpermps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1458407942, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 223, 22, 12, 253, 6, 138, 237, 86], OperandSize::Dword)
}

#[test]
fn vpermps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 66, 29, 205, 22, 233], OperandSize::Qword)
}

#[test]
fn vpermps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 913147690, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 117, 193, 22, 132, 81, 42, 135, 109, 54], OperandSize::Qword)
}

#[test]
fn vpermps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 839927249, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 53, 215, 22, 36, 117, 209, 69, 16, 50], OperandSize::Qword)
}

