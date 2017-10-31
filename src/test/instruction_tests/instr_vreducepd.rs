use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vreducepd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 137, 86, 243, 30], OperandSize::Dword)
}

#[test]
fn vreducepd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 930998909, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 139, 86, 164, 214, 125, 234, 125, 55, 92], OperandSize::Dword)
}

#[test]
fn vreducepd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 155, 86, 44, 126, 36], OperandSize::Dword)
}

#[test]
fn vreducepd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM31)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 3, 253, 143, 86, 215, 104], OperandSize::Qword)
}

#[test]
fn vreducepd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM17)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 975370236, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 227, 253, 142, 86, 12, 189, 252, 247, 34, 58, 16], OperandSize::Qword)
}

#[test]
fn vreducepd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM8)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Two, 1566826701, Some(OperandSize::Qword), None)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 115, 253, 156, 86, 132, 65, 205, 224, 99, 93, 1], OperandSize::Qword)
}

#[test]
fn vreducepd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 173, 86, 234, 94], OperandSize::Dword)
}

#[test]
fn vreducepd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 1934940814, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 169, 86, 172, 178, 142, 218, 84, 115, 25], OperandSize::Dword)
}

#[test]
fn vreducepd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Qword), None)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 185, 86, 20, 190, 67], OperandSize::Dword)
}

#[test]
fn vreducepd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM16)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 51, 253, 172, 86, 224, 83], OperandSize::Qword)
}

#[test]
fn vreducepd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM18)), operand2: Some(IndirectDisplaced(RBX, 524905633, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 227, 253, 174, 86, 147, 161, 108, 73, 31, 126], OperandSize::Qword)
}

#[test]
fn vreducepd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM29)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 99, 253, 191, 86, 40, 18], OperandSize::Qword)
}

#[test]
fn vreducepd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 159, 86, 220, 110], OperandSize::Dword)
}

#[test]
fn vreducepd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Two, 1426668401, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 207, 86, 140, 73, 113, 59, 9, 85, 45], OperandSize::Dword)
}

#[test]
fn vreducepd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM4)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 223, 86, 38, 39], OperandSize::Dword)
}

#[test]
fn vreducepd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 115, 253, 156, 86, 227, 119], OperandSize::Qword)
}

#[test]
fn vreducepd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM15)), operand2: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 115, 253, 207, 86, 62, 104], OperandSize::Qword)
}

#[test]
fn vreducepd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectScaledIndexed(RBX, RAX, Four, Some(OperandSize::Qword), None)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 253, 217, 86, 12, 131, 112], OperandSize::Qword)
}

