use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfixupimmpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(86)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 141, 84, 252, 86], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 819567515, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(9)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 229, 142, 84, 12, 141, 155, 155, 217, 48, 9], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 221, 159, 84, 31, 39], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(6)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 115, 245, 135, 84, 230, 6], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 229, 140, 84, 48, 1], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Qword), None)), operand4: Some(Literal8(42)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 229, 156, 84, 28, 242, 42], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(122)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 245, 175, 84, 228, 122], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1911336267, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(79)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 171, 84, 4, 197, 75, 173, 236, 113, 79], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 237, 185, 84, 59, 117], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM18)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 179, 149, 171, 84, 242, 59], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectDisplaced(RDI, 1203447015, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(2)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 115, 189, 172, 84, 191, 231, 36, 187, 71, 2], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM18)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: Some(Literal8(45)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 227, 237, 181, 84, 47, 45], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(0)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 205, 158, 84, 240, 0], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(64)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 205, 203, 84, 57, 64], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 1116023128, Some(OperandSize::Qword), None)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 205, 220, 84, 132, 215, 88, 41, 133, 66, 21], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM8)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 195, 237, 146, 84, 240, 34], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(29)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 197, 206, 84, 42, 29], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM18)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(80)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 237, 215, 84, 26, 80], OperandSize::Qword)
}

