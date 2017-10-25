use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vptest_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 23, 247], OperandSize::Dword)
}

fn vptest_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 41313802, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 23, 52, 157, 10, 102, 118, 2], OperandSize::Dword)
}

fn vptest_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 23, 218], OperandSize::Qword)
}

fn vptest_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 23, 36, 147], OperandSize::Qword)
}

fn vptest_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 23, 223], OperandSize::Dword)
}

fn vptest_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 1327541775, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 23, 164, 119, 15, 174, 32, 79], OperandSize::Dword)
}

fn vptest_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 23, 240], OperandSize::Qword)
}

fn vptest_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTEST, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1492120835, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 23, 12, 157, 3, 245, 239, 88], OperandSize::Qword)
}

