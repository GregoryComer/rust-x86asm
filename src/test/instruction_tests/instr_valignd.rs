use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn valignd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(35)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 85, 139, 3, 218, 35], OperandSize::Dword)
}

#[test]
fn valignd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 912086127, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(16)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 101, 140, 3, 132, 208, 111, 84, 93, 54, 16], OperandSize::Dword)
}

#[test]
fn valignd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 85, 154, 3, 52, 88, 101], OperandSize::Dword)
}

#[test]
fn valignd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM24)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 147, 37, 138, 3, 200, 34], OperandSize::Qword)
}

#[test]
fn valignd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectDisplaced(RCX, 407266041, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 115, 61, 131, 3, 153, 249, 98, 70, 24, 72], OperandSize::Qword)
}

#[test]
fn valignd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(67)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 45, 148, 3, 52, 247, 67], OperandSize::Qword)
}

#[test]
fn valignd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(49)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 109, 172, 3, 242, 49], OperandSize::Dword)
}

#[test]
fn valignd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 85, 172, 3, 51, 1], OperandSize::Dword)
}

#[test]
fn valignd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 352703240, Some(OperandSize::Dword), None)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 69, 186, 3, 28, 253, 8, 211, 5, 21, 119], OperandSize::Dword)
}

#[test]
fn valignd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM22)), operand4: Some(Literal8(10)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 163, 125, 173, 3, 222, 10], OperandSize::Qword)
}

#[test]
fn valignd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 1282519480, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 99, 101, 162, 3, 172, 248, 184, 177, 113, 76, 95], OperandSize::Qword)
}

#[test]
fn valignd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(9)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 13, 183, 3, 44, 82, 9], OperandSize::Qword)
}

#[test]
fn valignd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM4)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 69, 206, 3, 252, 83], OperandSize::Dword)
}

#[test]
fn valignd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 101, 201, 3, 12, 78, 65], OperandSize::Dword)
}

#[test]
fn valignd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Dword), None)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 117, 218, 3, 60, 152, 89], OperandSize::Dword)
}

#[test]
fn valignd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM15)), operand4: Some(Literal8(16)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 83, 85, 194, 3, 207, 16], OperandSize::Qword)
}

#[test]
fn valignd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 423382484, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 99, 45, 198, 3, 12, 85, 212, 77, 60, 25, 56], OperandSize::Qword)
}

#[test]
fn valignd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 856711933, Some(OperandSize::Dword), None)), operand4: Some(Literal8(98)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 53, 210, 3, 4, 133, 253, 98, 16, 51, 98], OperandSize::Qword)
}

