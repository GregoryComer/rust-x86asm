use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 22, 241], OperandSize::Dword)
}

#[test]
fn vpermps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EBX, 1411837387, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 22, 179, 203, 237, 38, 84], OperandSize::Dword)
}

#[test]
fn vpermps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 22, 232], OperandSize::Qword)
}

#[test]
fn vpermps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 22, 18], OperandSize::Qword)
}

#[test]
fn vpermps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 171, 22, 203], OperandSize::Dword)
}

#[test]
fn vpermps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 175, 22, 63], OperandSize::Dword)
}

#[test]
fn vpermps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(ESI, 507294615, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 186, 22, 174, 151, 179, 60, 30], OperandSize::Dword)
}

#[test]
fn vpermps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 66, 125, 164, 22, 231], OperandSize::Qword)
}

#[test]
fn vpermps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM20)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 93, 165, 22, 42], OperandSize::Qword)
}

#[test]
fn vpermps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 191, 22, 47], OperandSize::Qword)
}

#[test]
fn vpermps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 22, 228], OperandSize::Dword)
}

#[test]
fn vpermps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 2008890737, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 203, 22, 52, 181, 113, 61, 189, 119], OperandSize::Dword)
}

#[test]
fn vpermps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(ESI, 856557791, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 218, 22, 134, 223, 8, 14, 51], OperandSize::Dword)
}

#[test]
fn vpermps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 2, 85, 199, 22, 228], OperandSize::Qword)
}

#[test]
fn vpermps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 1214942678, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 125, 206, 22, 156, 146, 214, 141, 106, 72], OperandSize::Qword)
}

#[test]
fn vpermps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPS, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(RSI, 1493486491, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 93, 221, 22, 190, 155, 203, 4, 89], OperandSize::Qword)
}

