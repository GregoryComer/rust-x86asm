use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn roundpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 9, 236, 52], OperandSize::Dword)
}

fn roundpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 1360392958, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(78)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 9, 188, 243, 254, 242, 21, 81, 78], OperandSize::Dword)
}

fn roundpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 9, 201, 85], OperandSize::Qword)
}

fn roundpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RCX, 553976998, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 9, 177, 166, 4, 5, 33, 17], OperandSize::Qword)
}

