use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 232, 214], OperandSize::Dword)
}

#[test]
fn vpsubsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1398991158, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 232, 20, 181, 54, 233, 98, 83], OperandSize::Dword)
}

#[test]
fn vpsubsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 232, 222], OperandSize::Qword)
}

#[test]
fn vpsubsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 232, 10], OperandSize::Qword)
}

#[test]
fn vpsubsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 232, 228], OperandSize::Dword)
}

#[test]
fn vpsubsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 232, 48], OperandSize::Dword)
}

#[test]
fn vpsubsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 232, 208], OperandSize::Qword)
}

#[test]
fn vpsubsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 1297479798, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 232, 164, 198, 118, 248, 85, 77], OperandSize::Qword)
}

#[test]
fn vpsubsb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 137, 232, 205], OperandSize::Dword)
}

#[test]
fn vpsubsb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 117, 138, 232, 33], OperandSize::Dword)
}

#[test]
fn vpsubsb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 61, 140, 232, 218], OperandSize::Qword)
}

#[test]
fn vpsubsb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1739694940, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 21, 139, 232, 60, 197, 92, 163, 177, 103], OperandSize::Qword)
}

#[test]
fn vpsubsb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 172, 232, 210], OperandSize::Dword)
}

#[test]
fn vpsubsb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 101, 169, 232, 4, 200], OperandSize::Dword)
}

#[test]
fn vpsubsb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 65, 93, 167, 232, 233], OperandSize::Qword)
}

#[test]
fn vpsubsb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 886844660, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 13, 162, 232, 164, 182, 244, 44, 220, 52], OperandSize::Qword)
}

#[test]
fn vpsubsb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 201, 232, 204], OperandSize::Dword)
}

#[test]
fn vpsubsb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Four, 182244086, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 69, 207, 232, 180, 184, 246, 210, 220, 10], OperandSize::Dword)
}

#[test]
fn vpsubsb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM15)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 5, 204, 232, 204], OperandSize::Qword)
}

#[test]
fn vpsubsb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSB, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1259669894, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 53, 205, 232, 4, 141, 134, 9, 21, 75], OperandSize::Qword)
}

