use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vaesenc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENC, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 220, 198], OperandSize::Dword)
}

fn vaesenc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENC, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 220, 7], OperandSize::Dword)
}

fn vaesenc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENC, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 220, 200], OperandSize::Qword)
}

fn vaesenc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENC, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 1425141855, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 220, 188, 66, 95, 240, 241, 84], OperandSize::Qword)
}

