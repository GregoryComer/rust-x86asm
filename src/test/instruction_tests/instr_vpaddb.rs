use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 252, 226], OperandSize::Dword)
}

#[test]
fn vpaddb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 252, 14], OperandSize::Dword)
}

#[test]
fn vpaddb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 252, 208], OperandSize::Qword)
}

#[test]
fn vpaddb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 252, 60, 250], OperandSize::Qword)
}

#[test]
fn vpaddb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 252, 218], OperandSize::Dword)
}

#[test]
fn vpaddb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 252, 27], OperandSize::Dword)
}

#[test]
fn vpaddb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 252, 237], OperandSize::Qword)
}

#[test]
fn vpaddb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 252, 44, 152], OperandSize::Qword)
}

#[test]
fn vpaddb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 141, 252, 198], OperandSize::Dword)
}

#[test]
fn vpaddb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 2094174353, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 142, 252, 4, 221, 145, 144, 210, 124], OperandSize::Dword)
}

#[test]
fn vpaddb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 1, 109, 141, 252, 244], OperandSize::Qword)
}

#[test]
fn vpaddb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM13)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 21, 138, 252, 63], OperandSize::Qword)
}

#[test]
fn vpaddb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 170, 252, 248], OperandSize::Dword)
}

#[test]
fn vpaddb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 712286534, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 170, 252, 148, 208, 70, 161, 116, 42], OperandSize::Dword)
}

#[test]
fn vpaddb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 53, 161, 252, 250], OperandSize::Qword)
}

#[test]
fn vpaddb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 725669609, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 167, 252, 164, 192, 233, 214, 64, 43], OperandSize::Qword)
}

