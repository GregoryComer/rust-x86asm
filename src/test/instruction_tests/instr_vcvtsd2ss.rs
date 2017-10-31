use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtsd2ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 90, 193], OperandSize::Dword)
}

#[test]
fn vcvtsd2ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1488847327, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 90, 4, 117, 223, 1, 190, 88], OperandSize::Dword)
}

#[test]
fn vcvtsd2ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 90, 243], OperandSize::Qword)
}

#[test]
fn vcvtsd2ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 2042152934, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 90, 148, 159, 230, 199, 184, 121], OperandSize::Qword)
}

#[test]
fn vcvtsd2ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 247, 253, 90, 223], OperandSize::Dword)
}

#[test]
fn vcvtsd2ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 207, 138, 90, 36, 215], OperandSize::Dword)
}

#[test]
fn vcvtsd2ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 231, 242, 90, 254], OperandSize::Qword)
}

#[test]
fn vcvtsd2ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 231, 139, 90, 15], OperandSize::Qword)
}

