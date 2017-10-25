use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 235, 243], OperandSize::Dword)
}

fn vpor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Eight, 497974116, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 235, 172, 248, 100, 123, 174, 29], OperandSize::Dword)
}

fn vpor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 235, 225], OperandSize::Qword)
}

fn vpor_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 1043333089, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 235, 188, 146, 225, 255, 47, 62], OperandSize::Qword)
}

fn vpor_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 235, 253], OperandSize::Dword)
}

fn vpor_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Four, 2019651834, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 235, 180, 176, 250, 112, 97, 120], OperandSize::Dword)
}

fn vpor_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 235, 253], OperandSize::Qword)
}

fn vpor_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 235, 54], OperandSize::Qword)
}

