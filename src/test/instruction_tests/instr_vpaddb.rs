use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 252, 207], OperandSize::Dword)
}

#[test]
fn vpaddb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EDX, 514683942, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 252, 154, 38, 116, 173, 30], OperandSize::Dword)
}

#[test]
fn vpaddb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 252, 218], OperandSize::Qword)
}

#[test]
fn vpaddb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 2050521123, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 252, 164, 112, 35, 120, 56, 122], OperandSize::Qword)
}

#[test]
fn vpaddb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 252, 208], OperandSize::Dword)
}

#[test]
fn vpaddb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 252, 24], OperandSize::Dword)
}

#[test]
fn vpaddb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 252, 206], OperandSize::Qword)
}

#[test]
fn vpaddb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 252, 38], OperandSize::Qword)
}

#[test]
fn vpaddb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 139, 252, 248], OperandSize::Dword)
}

#[test]
fn vpaddb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 1116869901, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 69, 137, 252, 148, 215, 13, 21, 146, 66], OperandSize::Dword)
}

#[test]
fn vpaddb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 45, 142, 252, 235], OperandSize::Qword)
}

#[test]
fn vpaddb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1582907634, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 37, 134, 252, 44, 125, 242, 64, 89, 94], OperandSize::Qword)
}

#[test]
fn vpaddb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 172, 252, 199], OperandSize::Dword)
}

#[test]
fn vpaddb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 1887087115, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 170, 252, 188, 80, 11, 170, 122, 112], OperandSize::Dword)
}

#[test]
fn vpaddb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM23)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 49, 69, 165, 252, 213], OperandSize::Qword)
}

#[test]
fn vpaddb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectDisplaced(RBX, 959871870, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 5, 175, 252, 131, 126, 123, 54, 57], OperandSize::Qword)
}

