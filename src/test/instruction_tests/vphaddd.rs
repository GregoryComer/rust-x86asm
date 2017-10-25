use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vphaddd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 2, 212], OperandSize::Dword)
}

fn vphaddd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Two, 1985073862, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 2, 172, 73, 198, 210, 81, 118], OperandSize::Dword)
}

fn vphaddd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 2, 203], OperandSize::Qword)
}

fn vphaddd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 2, 31], OperandSize::Qword)
}

fn vphaddd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 2, 192], OperandSize::Dword)
}

fn vphaddd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EDI, 1531484866, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 2, 143, 194, 154, 72, 91], OperandSize::Dword)
}

fn vphaddd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 2, 250], OperandSize::Qword)
}

fn vphaddd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 2, 48], OperandSize::Qword)
}

