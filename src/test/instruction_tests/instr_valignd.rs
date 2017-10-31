use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn valignd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(2)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 93, 139, 3, 218, 2], OperandSize::Dword)
}

#[test]
fn valignd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1256379, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(29)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 101, 141, 3, 44, 197, 187, 43, 19, 0, 29], OperandSize::Dword)
}

#[test]
fn valignd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDX, 925151261, Some(OperandSize::Dword), None)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 69, 156, 3, 178, 29, 176, 36, 55, 66], OperandSize::Dword)
}

#[test]
fn valignd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM21)), operand4: Some(Literal8(120)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 51, 85, 143, 3, 197, 120], OperandSize::Qword)
}

#[test]
fn valignd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM28)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(24)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 115, 29, 130, 3, 26, 24], OperandSize::Qword)
}

#[test]
fn valignd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 466032883, Some(OperandSize::Dword), None)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 93, 146, 3, 60, 221, 243, 24, 199, 27, 78], OperandSize::Qword)
}

#[test]
fn valignd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 85, 170, 3, 248, 51], OperandSize::Dword)
}

#[test]
fn valignd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(ESI, 1703132429, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 101, 174, 3, 134, 13, 189, 131, 101, 34], OperandSize::Dword)
}

#[test]
fn valignd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(77)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 117, 186, 3, 55, 77], OperandSize::Dword)
}

#[test]
fn valignd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM15)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 83, 37, 171, 3, 223, 118], OperandSize::Qword)
}

#[test]
fn valignd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM29)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(23)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 99, 21, 161, 3, 35, 23], OperandSize::Qword)
}

#[test]
fn valignd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM19)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(24)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 101, 181, 3, 30, 24], OperandSize::Qword)
}

#[test]
fn valignd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 101, 205, 3, 216, 83], OperandSize::Dword)
}

#[test]
fn valignd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1522401594, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(0)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 117, 205, 3, 52, 245, 58, 1, 190, 90, 0], OperandSize::Dword)
}

#[test]
fn valignd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1998379140, Some(OperandSize::Dword), None)), operand4: Some(Literal8(15)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 218, 3, 44, 221, 132, 216, 28, 119, 15], OperandSize::Dword)
}

#[test]
fn valignd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM26)), operand3: Some(Direct(ZMM18)), operand4: Some(Literal8(104)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 179, 45, 198, 3, 202, 104], OperandSize::Qword)
}

#[test]
fn valignd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 603414111, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 115, 53, 196, 3, 172, 80, 95, 94, 247, 35, 81], OperandSize::Qword)
}

#[test]
fn valignd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM24)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 227, 61, 214, 3, 8, 51], OperandSize::Qword)
}

