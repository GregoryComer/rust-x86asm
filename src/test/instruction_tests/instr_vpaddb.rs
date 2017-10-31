use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 252, 234], OperandSize::Dword)
}

#[test]
fn vpaddb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1884405230, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 252, 28, 141, 238, 189, 81, 112], OperandSize::Dword)
}

#[test]
fn vpaddb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 252, 242], OperandSize::Qword)
}

#[test]
fn vpaddb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 252, 20, 65], OperandSize::Qword)
}

#[test]
fn vpaddb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 252, 236], OperandSize::Dword)
}

#[test]
fn vpaddb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(ECX, 1160922432, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 252, 145, 64, 69, 50, 69], OperandSize::Dword)
}

#[test]
fn vpaddb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 252, 206], OperandSize::Qword)
}

#[test]
fn vpaddb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 252, 52, 198], OperandSize::Qword)
}

#[test]
fn vpaddb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 141, 252, 250], OperandSize::Dword)
}

#[test]
fn vpaddb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 69, 143, 252, 10], OperandSize::Dword)
}

#[test]
fn vpaddb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 5, 133, 252, 248], OperandSize::Qword)
}

#[test]
fn vpaddb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectDisplaced(RDI, 1631314051, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 45, 142, 252, 191, 131, 224, 59, 97], OperandSize::Qword)
}

#[test]
fn vpaddb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 172, 252, 196], OperandSize::Dword)
}

#[test]
fn vpaddb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Eight, 408921803, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 171, 252, 156, 203, 203, 166, 95, 24], OperandSize::Dword)
}

#[test]
fn vpaddb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM23)), operand3: Some(Direct(YMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 193, 69, 165, 252, 233], OperandSize::Qword)
}

#[test]
fn vpaddb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 957523153, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 117, 162, 252, 4, 245, 209, 164, 18, 57], OperandSize::Qword)
}

