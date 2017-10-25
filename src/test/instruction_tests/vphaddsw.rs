use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vphaddsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 3, 250], OperandSize::Dword)
}

fn vphaddsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 875106034, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 3, 156, 66, 242, 14, 41, 52], OperandSize::Dword)
}

fn vphaddsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 3, 203], OperandSize::Qword)
}

fn vphaddsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1271054810, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 3, 44, 69, 218, 193, 194, 75], OperandSize::Qword)
}

fn vphaddsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 3, 214], OperandSize::Dword)
}

fn vphaddsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 3, 47], OperandSize::Dword)
}

fn vphaddsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 3, 228], OperandSize::Qword)
}

fn vphaddsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 3, 44, 200], OperandSize::Qword)
}

