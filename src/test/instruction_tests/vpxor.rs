use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpxor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 239, 230], OperandSize::Dword)
}

fn vpxor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Eight, 2013244843, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 239, 132, 246, 171, 173, 255, 119], OperandSize::Dword)
}

fn vpxor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 239, 231], OperandSize::Qword)
}

fn vpxor_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 523638493, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 239, 36, 69, 221, 22, 54, 31], OperandSize::Qword)
}

fn vpxor_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 239, 197], OperandSize::Dword)
}

fn vpxor_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 504873840, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 239, 156, 178, 112, 195, 23, 30], OperandSize::Dword)
}

fn vpxor_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 239, 245], OperandSize::Qword)
}

fn vpxor_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 848752970, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 239, 188, 83, 74, 241, 150, 50], OperandSize::Qword)
}

