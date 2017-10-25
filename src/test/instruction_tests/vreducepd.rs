use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vreducepd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 137, 86, 240, 122], OperandSize::Dword)
}

fn vreducepd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 468729885, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 137, 86, 36, 125, 29, 64, 240, 27, 86], OperandSize::Dword)
}

fn vreducepd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 862968910, Some(OperandSize::Qword), None)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 156, 86, 28, 189, 78, 220, 111, 51, 111], OperandSize::Dword)
}

fn vreducepd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM24)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 131, 253, 139, 86, 248, 63], OperandSize::Qword)
}

fn vreducepd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM28)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 408001384, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 99, 253, 138, 86, 36, 245, 104, 155, 81, 24, 14], OperandSize::Qword)
}

fn vreducepd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RSI, 467117476, Some(OperandSize::Qword), None)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 154, 86, 158, 164, 165, 215, 27, 84], OperandSize::Qword)
}

fn vreducepd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 173, 86, 223, 96], OperandSize::Dword)
}

fn vreducepd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(EBX, 901731936, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 174, 86, 139, 96, 86, 191, 53, 32], OperandSize::Dword)
}

fn vreducepd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectDisplaced(ESI, 209981931, Some(OperandSize::Qword), None)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 189, 86, 150, 235, 17, 132, 12, 93], OperandSize::Dword)
}

fn vreducepd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 172, 86, 212, 88], OperandSize::Qword)
}

fn vreducepd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM19)), operand2: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 227, 253, 170, 86, 30, 56], OperandSize::Qword)
}

fn vreducepd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 189, 86, 49, 98], OperandSize::Qword)
}

fn vreducepd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 154, 86, 251, 101], OperandSize::Dword)
}

fn vreducepd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexed(EDX, ECX, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 206, 86, 28, 202, 76], OperandSize::Dword)
}

fn vreducepd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 287212325, Some(OperandSize::Qword), None)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 222, 86, 12, 253, 37, 131, 30, 17, 8], OperandSize::Dword)
}

fn vreducepd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM29)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 19, 253, 153, 86, 245, 24], OperandSize::Qword)
}

fn vreducepd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM10)), operand2: Some(IndirectScaledIndexed(RDI, RDI, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 115, 253, 203, 86, 20, 255, 7], OperandSize::Qword)
}

fn vreducepd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM12)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 583097052, Some(OperandSize::Qword), None)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 253, 217, 86, 164, 193, 220, 90, 193, 34, 108], OperandSize::Qword)
}

