use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 254, 205], OperandSize::Dword)
}

#[test]
fn vpaddd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 254, 17], OperandSize::Dword)
}

#[test]
fn vpaddd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 254, 207], OperandSize::Qword)
}

#[test]
fn vpaddd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1679116881, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 254, 44, 133, 81, 74, 21, 100], OperandSize::Qword)
}

#[test]
fn vpaddd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 254, 196], OperandSize::Dword)
}

#[test]
fn vpaddd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 254, 52, 73], OperandSize::Dword)
}

#[test]
fn vpaddd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 254, 239], OperandSize::Qword)
}

#[test]
fn vpaddd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 254, 36, 81], OperandSize::Qword)
}

#[test]
fn vpaddd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 139, 254, 255], OperandSize::Dword)
}

#[test]
fn vpaddd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 142, 254, 4, 176], OperandSize::Dword)
}

#[test]
fn vpaddd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 117, 159, 254, 4, 123], OperandSize::Dword)
}

#[test]
fn vpaddd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 77, 134, 254, 241], OperandSize::Qword)
}

#[test]
fn vpaddd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 85, 137, 254, 22], OperandSize::Qword)
}

#[test]
fn vpaddd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 53, 159, 254, 20, 201], OperandSize::Qword)
}

